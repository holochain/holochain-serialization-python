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
    assert data == bytes([239, 242, 21, 207, 225, 142, 252, 4, 173, 215, 48, 207, 89, 56, 92, 154, 189, 144, 184, 122, 247, 4, 123, 250, 167, 5, 75, 90, 193, 8, 118, 79, 132, 141, 111, 224, 95, 195, 191, 72, 198, 154, 80, 25, 124, 22, 111, 20, 217, 179, 236, 78, 185, 127, 128, 72, 63, 1, 234, 52, 0, 91, 166, 225]), f"got {list(data)}"

    cap_secret = [0] * 64
    zome_call_unsigned = holochain_serialization.ZomeCallUnsignedPy(provenance, dna_hash, agent_pub_key, zome_name, fn_name, payload, nonce, expires_at, cap_secret=cap_secret)
    data = holochain_serialization.get_data_to_sign(zome_call_unsigned)
    assert data == bytes([167, 218, 30, 74, 112, 49, 99, 22, 176, 72, 224, 240, 60, 199, 93, 137, 15, 220, 162, 254, 73, 117, 174, 56, 181, 254, 197, 16, 103, 111, 109, 92, 212, 176, 122, 251, 49, 238, 1, 38, 250, 191, 221, 139, 12, 40, 114, 30, 147, 254, 131, 94, 23, 226, 112, 236, 115, 124, 223, 204, 102, 147, 93, 210]), f"got {list(data)}"

    print("Passed!")

