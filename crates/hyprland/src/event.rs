use crate::error::EventParseError;

pub trait Event: Sized + Send + Sync + 'static {
    const NAME: &'static str;
    type Data: Send + Sync + 'static;

    fn parse_data(elements: &[&str]) -> Result<Self::Data, EventParseError>;
}

macro_rules! count_idents {
    ($a:ident) => {
        1
    };
    ($a:ident, $($b:ident),+) => {
        1 + count_idents!($($b),*)
    };
    () => {
        0
    };
}

macro_rules! event {
    (
        $name:literal as $event:ident >> {
            $($field:ident:$ty:ty),* $(,)?
        } as $data:ident
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $data {
            $(pub $field: $ty,)*
        }

        pub enum $event {}

        impl Event for $event {
            const NAME: &'static str = $name;
            type Data = $data;

            fn parse_data(elements: &[&str]) -> Result<Self::Data, EventParseError> {
                if elements.len() != count_idents!($($field),*) {
                    return Err(EventParseError::InvalidData);
                }

                let mut iter = elements.iter();

                Ok($data {
                    $(
                        $field: iter
                            .next()
                            .ok_or(EventParseError::MissingField(stringify!($field)))?
                            .parse()
                            .map_err(|_| EventParseError::ParseFailed(stringify!($field)))?,
                    )*
                })
            }
        }
    };

    (
        $(
            $name:literal as $event:ident >> {
                $($field:ident:$ty:ty),* $(,)?
            } as $data:ident
        ),* $(,)?
    ) => {
        $(
            event!($name as $event >> { $($field: $ty),* } as $data);
        )*
    };
}

event!(
    "workspace" as Workspace >> {
        name: String
    } as WorkspaceData,
    "activewindow" as ActiveWindow >> {
        class: String,
        title: String,
    } as ActiveWindowData,
    "openwindow" as OpenWindow >> {
        address: String,
        name: String,
        class: String,
        title: String,
    } as OpenWindowData,
    "closewindow" as CloseWindow >> {
        address: String,
    } as CloseWindowData,
    "movewindow" as MoveWindow >> {
        address: String,
        workspace: String,
    } as MoveWindowData,
);
