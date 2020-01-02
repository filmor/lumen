use super::*;

#[test]
fn with_integer_without_byte_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    byte(arc_process.clone()),
                    is_integer_is_not_byte(arc_process.clone()),
                )
                    .prop_map(|(head, tail)| (arc_process.cons(head, tail).unwrap(), tail)),
                |(iolist, element)| {
                    prop_assert_badarg!(
                        native(&arc_process, iolist),
                        format!(
                            "iolist ({}) element ({}) is not a byte, binary, or nested iolist",
                            iolist, element
                        )
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_empty_list_returns_1_byte_binary() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &any::<u8>().prop_map(|byte| {
                    (
                        arc_process
                            .cons(arc_process.integer(byte).unwrap(), Term::NIL)
                            .unwrap(),
                        byte,
                    )
                }),
                |(list, byte)| {
                    let binary = arc_process.binary_from_bytes(&[byte]).unwrap();

                    prop_assert_eq!(native(&arc_process, list), Ok(binary));

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_byte_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(byte(arc_process.clone()), byte(arc_process.clone())),
                |(head, tail)| {
                    let iolist = arc_process.cons(head, tail).unwrap();

                    prop_assert_badarg!(
                        native(&arc_process, iolist),
                        format!("iolist ({}) tail ({}) cannot be a byte", iolist, tail)
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_list_without_byte_tail_returns_binary() {
    with(|head_byte, head, process| {
        let tail_head_byte = 254;
        let tail_head = process.integer(tail_head_byte).unwrap();

        let tail_tail = Term::NIL;

        let tail = process.cons(tail_head, tail_tail).unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            native(process, iolist),
            Ok(process
                .binary_from_bytes(&[head_byte, tail_head_byte],)
                .unwrap())
        );
    })
}

#[test]
fn with_heap_binary_returns_binary() {
    with(|head_byte, head, process| {
        let tail = process.binary_from_bytes(&[1, 2]).unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            native(process, iolist),
            Ok(process.binary_from_bytes(&[head_byte, 1, 2]).unwrap())
        );
    })
}

#[test]
fn with_subbinary_without_bitcount_returns_binary() {
    with(|head_byte, head, process| {
        let original = process
            .binary_from_bytes(&[0b0111_1111, 0b1000_0000])
            .unwrap();
        let tail = process
            .subbinary_from_original(original, 0, 1, 1, 0)
            .unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            native(process, iolist),
            Ok(process.binary_from_bytes(&[head_byte, 255]).unwrap())
        );
    })
}

#[test]
fn with_subbinary_with_bitcount_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    byte(arc_process.clone()),
                    strategy::term::binary::sub::is_not_binary(arc_process.clone()),
                ),
                |(head, tail)| {
                    let iolist = arc_process.cons(head, tail).unwrap();

                    prop_assert_badarg!(
                        native(&arc_process, iolist),
                        format!(
                            "iolist ({}) element ({}) is not a byte, binary, or nested iolist",
                            iolist, tail
                        )
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn with<F>(f: F)
where
    F: FnOnce(u8, Term, &Process) -> (),
{
    with_process(|process| {
        let head_byte: u8 = 0;
        let head = process.integer(head_byte).unwrap();

        f(head_byte, head, &process);
    })
}
