use siphasher::sip::SipHasher13;
use std::hash::Hasher;

/// Hyperliquid TID = upper 50 bits of SipHash-1-3( (buyer_oid, seller_oid), key = 0 ).
pub fn compute_tid(buyer_oid: u64, seller_oid: u64) -> u64 {
    // Fixed 128-bit key (k0 = 0, k1 = 0) as used on-chain
    let mut hasher = SipHasher13::new_with_keys(0, 0);

    // Feed (buyer_oid, seller_oid) in little-endian order
    hasher.write_u64(buyer_oid);
    hasher.write_u64(seller_oid);

    let full_hash = hasher.finish();
    full_hash >> 14 // keep the upper 50 bits
}

fn main() {
    // Expect two u64 arguments: buyer_oid and seller_oid
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <buyer_oid> <seller_oid>", args[0]);
        std::process::exit(1);
    }

    let buyer_oid: u64 = args[1]
        .parse()
        .expect("buyer_oid must be an unsigned 64-bit integer");
    let seller_oid: u64 = args[2]
        .parse()
        .expect("seller_oid must be an unsigned 64-bit integer");

    let tid = compute_tid(buyer_oid, seller_oid);
    println!("TID: {} (0x{:x})", tid, tid);
}

