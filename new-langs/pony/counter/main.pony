use "collections"

actor Counter
    var _count: U32

    new create() =>
        _count = 0

    be increment(main: Main) =>
        _count = _count + 1
        main.display(_count)

    be get_and_reset(main: Main) =>
        main.display(_count)
        _count = 0

actor Accumulator
    var name: String
    var _count: U32 = 5
    
    new create(name': String) =>
        name = name'

    fun ref get_name(): String => 
        name

    be accumulate(main: Main, counter: Counter) =>
        main.display_name(this, false)
        
        for i in Range[U32](0, _count) do
            counter.increment(main)
        end

        main.display_name(this, true)

actor Main
    var _env: Env

    new create(env: Env) =>
        _env = env

        var count: U32 = try env.args(1)?.u32()? else 10 end
        var counter = Counter

        for i in Range[U32](0, count) do
            counter.increment(this)
        end

        counter.get_and_reset(this)

        var num_acc: U32 = try env.args(2)?.u32()? else 5 end
        var num = 0
        for i in Range[U32](0, num_acc) do
            let name = "Agent" + num.string()
            num = num + 1
            var acc = Accumulator(name)

            acc.accumulate(this, counter)
        end

        counter.get_and_reset(this)

    be display(result: U32) =>
        _env.out.print(result.string())

    be display_name(acc: Accumulator, done: Bool) =>
        let name' = acc.get_name()
        
        match done
            | true => _env.out.print(name' + " is finished.")
            | false => _env.out.print("Accumulating with " + name')
        end
