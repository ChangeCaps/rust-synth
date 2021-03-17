#[macro_export]
macro_rules! node {
    ($ident:ident => $name:literal (&mut $self0:ident, $ctx:ident: &NodeCtx$(,$input:ident: $i_ty:ident)*) ->
        [$($output:ident: $o_ty:ident),*]
        $block:block

    $(
        fn setup(&mut $setup_self:ident) {

        }
    )?

    $(display $should_display:ident;)?

    fn ui(&mut $self:ident, $ui_param:ident: &mut Ui) -> bool $ui:block
) => {
        impl $crate::node::Node for $ident {
            fn name(&self) -> &str {
                $name
            }

            fn input_slot_types(&self) -> &[(&'static str, $crate::node::SlotType)] {
                &[$((stringify!($input), $crate::node::SlotType::$i_ty)),*]
            }

            fn output_slot_types(&self) -> &[(&'static str, $crate::node::SlotType)] {
                &[$((stringify!($output), $crate::node::SlotType::$o_ty)),*]
            }

            $(
                fn setup(&mut $setup_self) {

                }
            )?

            $(
                fn display_out(&self) -> &Option<&str> {
                    &Some(stringify!($should_display))
                }
            )?

            #[allow(unused_variables)]
            fn run(
                &$self0,
                $ctx: &$crate::node::NodeCtx,
                input: std::collections::HashMap<String, $crate::node::SlotValue>,
            ) -> Vec<(&'static str, $crate::node::SlotValue)> {
                $(
                    let $input = input[stringify!($input)];
                )*

                $(
                    let $output;
                )*

                $block

                vec![
                    $((stringify!($output), $output)),*
                ]
            }

            fn ui(
                &mut $self,
                $ui_param: &mut eframe::egui::Ui,
            ) -> bool {
                $ui
            }
        }
    };
}
