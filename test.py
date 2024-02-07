import holochain_serialization

if __name__ == "__main__":
    provenance = [132, 32, 36,] + [0] * 36
    dna_hash = [132, 45, 36] + [0] * 36
    agent_pub_key = [132, 32, 36,] + [0] * 36
    zome_name = "test"
    fn_name = "test"
    payload = [0] * 10
    nonce = [0] * 32
    expires_at = int(55) # Not valid, but want a predictable value for the hash
    
    zome_call_unsigned = holochain_serialization.ZomeCallUnsignedPy(provenance, dna_hash, agent_pub_key, zome_name, fn_name, payload, nonce, expires_at)
    data = holochain_serialization.get_data_to_sign(zome_call_unsigned)
    assert data == [16, 212, 254, 166, 94, 173, 226, 249, 219, 188, 104, 154, 154, 224, 156, 247, 195, 147, 157, 55, 16, 132, 51, 102, 178, 72, 130, 57, 125, 214, 200, 27]

    cap_secret = [0] * 64
    zome_call_unsigned = holochain_serialization.ZomeCallUnsignedPy(provenance, dna_hash, agent_pub_key, zome_name, fn_name, payload, nonce, expires_at, cap_secret=cap_secret)
    data = holochain_serialization.get_data_to_sign(zome_call_unsigned)
    assert data == [19, 82, 166, 180, 255, 212, 169, 179, 198, 226, 103, 225, 97, 67, 249, 4, 168, 8, 216, 29, 244, 79, 142, 141, 126, 57, 135, 6, 73, 75, 102, 250], f"got {data}"

    print("Passed!")

