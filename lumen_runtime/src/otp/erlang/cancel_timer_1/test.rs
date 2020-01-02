mod with_local_reference;

use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::otp::erlang;
use crate::otp::erlang::cancel_timer_1::native;
use crate::process::SchedulerDependentAlloc;
use crate::scheduler::{with_process, with_process_arc};
use crate::test::{has_message, receive_message, strategy, timeout_message};
use crate::time::Milliseconds;
use crate::{process, timer};

#[test]
fn without_reference_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_not_reference(arc_process.clone()),
                |timer_reference| {
                    prop_assert_badarg!(
                        native(&arc_process, timer_reference,),
                        format!(
                            "timer_reference ({}) is not a local reference",
                            timer_reference
                        )
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}
