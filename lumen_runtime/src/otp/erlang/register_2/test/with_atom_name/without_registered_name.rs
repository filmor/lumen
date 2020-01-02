use super::*;

use proptest::strategy::Strategy;

mod with_local_pid;

#[test]
fn without_pid_or_port_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::atom(),
                    strategy::term(arc_process.clone())
                        .prop_filter("Cannot be pid or port", |pid_or_port| {
                            !(pid_or_port.is_pid() || pid_or_port.is_port())
                        }),
                ),
                |(name, pid_or_port)| {
                    prop_assert_badarg!(
                        erlang::register_2::native(arc_process.clone(), name, pid_or_port),
                        format!("{} must be a local pid or port", pid_or_port)
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}
