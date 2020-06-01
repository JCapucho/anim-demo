use common::*;
use wasmtime::*;

fn main() {
    let before = std::time::Instant::now();

    // Create our `Store` with multi value so that animations return a pointer to the new skeleton and the rate
    let wasmtime_store = Store::new(&Engine::new(
        Config::new().cranelift_opt_level(OptLevel::SpeedAndSize),
    ));
    let module = Module::from_file(&wasmtime_store, "animations.wasm").unwrap();
    let instance = Instance::new(&module, &[]).unwrap();
    println!(
        "Instantiation: {}",
        std::time::Instant::now()
            .duration_since(before)
            .as_secs_f64()
    );

    let before = std::time::Instant::now();

    // Load up our exports from the instance
    let memory = instance
        .get_memory("memory")
        .expect("failed to find `memory` export");
    let get_staging_buffer_ptr_func = instance
        .get_func("get_staging_buffer_ptr")
        .expect("failed to find `get_staging_buffer_ptr` export")
        .get0::<i32>()
        .unwrap();
    let metadata_func = instance
        .get_func("metadata")
        .expect("failed to find `metadata` export")
        .get0::<i32>()
        .unwrap();
    let metadata: common::Metadata = {
        let ptr = metadata_func().unwrap() as usize;
        let staging = unsafe { &memory.data_unchecked()[ptr..ptr + 2048] };
        bincode::deserialize_from(staging).unwrap()
    };

    println!(
        "Get metadata: {}",
        std::time::Instant::now()
            .duration_since(before)
            .as_secs_f64()
    );

    println!("{:#?}", metadata);

    let before = std::time::Instant::now();
    let character_idle_func = instance
        .get_func("character_idle")
        .expect("failed to find `character_idle` export")
        .get1::<f64, i32>()
        .unwrap();

    unsafe {
        let ptr = get_staging_buffer_ptr_func().unwrap() as isize;
        let data =
            memory.data_ptr().offset(ptr) as *mut AnimationPassTrough<CharacterSkeleton, f64>;

        data.write(AnimationPassTrough {
            dependency: 0.0,
            skeleton: Default::default(),
            attr: Default::default(),
            rate: 3.5,
        });
    }

    let ret = unsafe {
        let ret = character_idle_func(0.0).unwrap();
        let ptr = ret as isize;
        let data =
            memory.data_ptr().offset(ptr) as *mut AnimationPassTrough<CharacterSkeleton, f64>;

        data.read()
    };

    println!(
        "Update skeleton: {}",
        std::time::Instant::now()
            .duration_since(before)
            .as_secs_f64()
    );

    println!("{:#?}", ret);
}
