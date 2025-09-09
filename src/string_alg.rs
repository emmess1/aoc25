//! String algorithms: KMP, Z-function, and a simple rolling hash.

/// KMP prefix (failure) function.
pub fn kmp_prefix(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut pi = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        while j>0 && s[i]!=s[j] { j = pi[j-1]; }
        if s[i]==s[j] { j+=1; }
        pi[i]=j;
    }
    pi
}

/// KMP search returns starting indices of pattern `p` in text `t`.
pub fn kmp_search(t: &str, p: &str) -> Vec<usize> {
    let (t, p) = (t.as_bytes(), p.as_bytes());
    if p.is_empty() { return (0..=t.len()).collect(); }
    let pi = kmp_prefix(p);
    let mut res = Vec::new();
    let mut j=0;
    for i in 0..t.len() {
        while j>0 && t[i]!=p[j] { j=pi[j-1]; }
        if t[i]==p[j] { j+=1; if j==p.len(){ res.push(i+1-j); j=pi[j-1]; } }
    }
    res
}

/// Z-function: z[i] = length of longest substring starting at i that is also a prefix.
pub fn z_function(s: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0,0);
    for i in 1..n {
        if i<r { z[i] = z[i-l].min(r - i); }
        while i+z[i] < n && s[z[i]] == s[i+z[i]] { z[i]+=1; }
        if i+z[i] > r { l=i; r=i+z[i]; }
    }
    z[0]=n; z
}

/// Simple rolling hash for fast substring hashing (base/mod pair).
#[derive(Clone, Debug)]
pub struct RollingHash {
    base: u64,
    modu: u64,
    pref: Vec<u64>,
    pow: Vec<u64>,
}
impl RollingHash {
    pub fn new(s: &str, base: u64, modu: u64) -> Self {
        let n = s.len();
        let mut pref = vec![0; n+1];
        let mut pow = vec![1; n+1];
        for (i,&b) in s.as_bytes().iter().enumerate() { pref[i+1] = (pref[i]*base + b as u64) % modu; pow[i+1] = (pow[i]*base) % modu; }
        Self { base, modu, pref, pow }
    }
    /// Hash of substring s[l..r) (0-based, exclusive r).
    pub fn hash(&self, l: usize, r: usize) -> u64 {
        (self.pref[r] + self.modu - (self.pref[l] * self.pow[r-l]) % self.modu) % self.modu
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kmp_and_z() {
        let t = "ababaabababa"; let p = "ababa";
        let occ = kmp_search(t,p);
        assert_eq!(occ, vec![0,5,7]);
        let z = z_function("aaaaa");
        assert_eq!(z, vec![5,4,3,2,1]);
    }
    #[test]
    fn roll_hash() {
        let s = "abcdefg"; let h = RollingHash::new(s, 911382323, 972663749);
        assert_eq!(h.hash(0,3), h.hash(0,3));
        assert_ne!(h.hash(0,3), h.hash(1,4));
    }
}

