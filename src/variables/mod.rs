use crate::prelude::*;
use astal_obj::*;

macro_rules! lateinit {
    ($static:ident, $fn:ident, $command:expr) => {
        static mut $static: Option<Variable<String>> = None;

        #[allow(static_mut_refs)]
        pub fn $fn() -> &'static Variable<String> {
            unsafe {
                if $static.is_none() {
                    $static = Some(Variable::from_astal(
                        AstalVariable::new(Box::leak(Box::new(Value::from("".to_string()))))
                            .pollv(1000, $command, None)
                            .unwrap(),
                    ));
                }

                $static.as_ref().unwrap()
            }
        }
    };

    ($($static:ident, $fn:ident, $command:expr);+) => {
        $(lateinit!($static, $fn, $command);)+
    }
}

lateinit!(
    DATE, date, &["date", "+%A, %b %d"];
    TIME, time, &["date", "+%-l:%M %p"]
);
