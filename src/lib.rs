use holo_hash::{AgentPubKey, DnaHash};
use holochain_zome_types::{prelude::ZomeCallUnsigned, CapSecret, CellId, ExternIO, Nonce256Bits, Timestamp, CAP_SECRET_BYTES};
use pyo3::prelude::*;

// A Python class to represent holochain_zome_types::ZomeCallUnsigned but with types that can be passed from Python to Rust easily. There is 
// a TryFrom implementation to convert this struct to the real ZomeCallUnsigned struct.
#[pyclass]
#[derive(Clone)]
struct ZomeCallUnsignedPy {
    provenance: Vec<u8>,
    cell_id_dna_hash: Vec<u8>,
    cell_id_agent_pub_key: Vec<u8>,
    zome_name: String,
    fn_name: String,
    cap_secret: Option<Vec<u8>>,
    payload: Vec<u8>,
    nonce: Vec<u8>,
    expires_at: i64,
}

#[pymethods]
impl ZomeCallUnsignedPy {
    /// Constructor for ZomeCallUnsignedPy that can be called like __init__ from Python.
    #[new]
    fn new(
        provenance: Vec<u8>,
        cell_id_dna_hash: Vec<u8>,
        cell_id_agent_pub_key: Vec<u8>,
        zome_name: String,
        fn_name: String,
        payload: Vec<u8>,
        nonce: Vec<u8>,
        expires_at: i64,
        cap_secret: Option<Vec<u8>>,
    ) -> Self {
        ZomeCallUnsignedPy {
            provenance,
            cell_id_dna_hash,
            cell_id_agent_pub_key,
            zome_name,
            fn_name,
            cap_secret,
            payload,
            nonce,
            expires_at,
        }
    }
}

impl TryFrom<ZomeCallUnsignedPy> for ZomeCallUnsigned {
    type Error = PyErr;

    fn try_from(zome_call_unsigned_py: ZomeCallUnsignedPy) -> PyResult<Self> {
        let provenance: holo_hash::HoloHash<holo_hash::hash_type::Agent> =
            AgentPubKey::from_raw_39(zome_call_unsigned_py.provenance.into())
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error converting agent key for provenance: {:?}", e))
                })?;

        let dna_hash = DnaHash::from_raw_39(zome_call_unsigned_py.cell_id_dna_hash.into())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error converting dna hash for cell id: {:?}", e)))?;

        let agent_pubkey =
            AgentPubKey::from_raw_39(zome_call_unsigned_py.cell_id_agent_pub_key.into())
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error converting agent pub key for cell id: {:?}", e))
                })?;

        let cap_secret: Option<CapSecret> = match zome_call_unsigned_py.cap_secret {
            None => None,
            Some(cap_secret_bytes) => {
                let sized_cap_secret: [u8; CAP_SECRET_BYTES] = cap_secret_bytes.as_slice().try_into().map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error converting cap secret: {:?}", e))
                })?;
                Some(sized_cap_secret.into())
            }
        };

        let sized_nonce: [u8; 32] = zome_call_unsigned_py.nonce.try_into().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error converting nonce: {:?}", e))
        })?;
        let nonce: Nonce256Bits = sized_nonce.into();

        Ok(ZomeCallUnsigned {
            provenance,
            cell_id: CellId::new(dna_hash, agent_pubkey),
            zome_name: zome_call_unsigned_py.zome_name.into(),
            fn_name: zome_call_unsigned_py.fn_name.into(),
            cap_secret,
            payload: ExternIO(zome_call_unsigned_py.payload),
            nonce,
            expires_at: Timestamp(zome_call_unsigned_py.expires_at),
        })
    }
}

/// Serialize the contents of a ZomeCallUnsigned to a byte array and then compute a hash of the result. The resulting byte array is returned.
/// 
/// You can find the Holochain implementation [here](https://github.com/holochain/holochain/blob/develop/crates/holochain_zome_types/src/zome_io.rs#L260) 
/// which is what this code is calling.
#[pyfunction]
fn get_data_to_sign(zome_call_unsigned: ZomeCallUnsignedPy) -> PyResult<Vec<u8>> {
    let zome_call_unsigned: ZomeCallUnsigned = zome_call_unsigned.try_into()?;
    let data_to_sign = zome_call_unsigned
        .data_to_sign()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {:?}", e)))?;
    Ok(data_to_sign.into_iter().cloned().collect())
}

/// Holochain serialization exposed as a Python module.
#[pymodule]
fn holochain_serialization(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ZomeCallUnsignedPy>()?;
    m.add_function(wrap_pyfunction!(get_data_to_sign, m)?)?;
    Ok(())
}
