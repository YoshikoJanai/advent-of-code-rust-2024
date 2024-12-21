

pub fn logical_xor(p: bool, q: bool) -> bool {
    (p || q) && (!p || !q)
}