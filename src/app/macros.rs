#[macro_export]
macro_rules! node {
    ($ident:ident => $name:literal (ctx: &NodeCtx, $($input:ident: $i_ty:ident),*) -> [$($output:ident: $o_ty:ident),*]
        $block:block

    fn ui(ui: &mut Ui) -> bool $ui:block
) => {
        impl $crate::node::Node for $ident {
            fn name(&self) -> &str {
                stringify!($ident)
            }

            fn input_slot_types(&self) -> &[(&'static str, $crate::node::SlotType)] {
                &[$((stringify!($input), $crate::node::SlotType::$i_ty)),*]
            }

            fn output_slot_types(&self) -> &[(&'static str, $crate::node::SlotType)] {
                &[$((stringify!($output), $crate::node::SlotType::$o_ty)),*]
            }

            fn run(
                &mut self,
                ctx: &$crate::node::NodeCtx,
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
                    $((stringify!($output), $output))*
                ]
            }

            fn ui(
                &mut self,
                ui: &mut eframe::egui::Ui,
            ) -> bool {
                $ui
            }
        }
    };
}
