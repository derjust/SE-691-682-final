    (module
        (func $foo)

        (func (export "bar")
            call $foo
        )
    )
