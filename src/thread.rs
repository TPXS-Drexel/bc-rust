/*
    let mut vec: Vec<&str> = Vec::with_capacity(LEADING_ZEROS * 3);
    for _ in 0..LEADING_ZEROS * 3 {
        vec.push(input);
    }
    //
    let mut lower = 0;
    let mut upper = MAX_NONCE / (LEADING_ZEROS as u64 * 3);
    let noncer = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in vec {
        let noncer = Arc::clone(&noncer);
        let handle = thread::spawn(move || {
            find_nonce(i, lower, upper)
        });
        
        handles.push(handle);
        
        let temp = lower;
        lower = upper;
        upper = upper + temp;
        
    }
    let mut nonce_test: u64 = 0;
    for handle in handles {
        handle.join().unwrap();
        
    }
    
    //println!("handle? {:?}", handles);
    println!("nonce: {}", *noncer.lock().unwrap());
    
    */
    
    let max_hash = BigUint::one() << (256 - 4 * LEADING_ZEROS); //>
    /*
    let workers = (0..100u64).map(|mut i| std::thread::spawn(move || loop {
        i = {
            let mut contents = Vec::new();
            contents.extend_from_slice(input.as_bytes());
            contents.extend_from_slice(&convert_u64_little_endian(i));
            let mut hasher = Sha256::new();
            hasher.input(&contents);
            let mut hash = Sha256Hash::default();
            hasher.result(&mut hash);
            let hash_int =u64::from_be_bytes(hash);
            hash_int
        }
    }));
    */

    
   

    //
    
    
    //println!("{:?}", nonce);