(function() {
    var type_impls = Object.fromEntries([["fastcrypto_zkp",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-VerifyingKey%3CG1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/mod.rs.html#23\">source</a><a href=\"#impl-Debug-for-VerifyingKey%3CG1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;G1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"fastcrypto/groups/trait.Pairing.html\" title=\"trait fastcrypto::groups::Pairing\">Pairing</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"fastcrypto_zkp/groth16/struct.VerifyingKey.html\" title=\"struct fastcrypto_zkp::groth16::VerifyingKey\">VerifyingKey</a>&lt;G1&gt;<div class=\"where\">where\n    G1::<a class=\"associatedtype\" href=\"fastcrypto/groups/trait.Pairing.html#associatedtype.Other\" title=\"type fastcrypto::groups::Pairing::Other\">Other</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/mod.rs.html#23\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","fastcrypto_zkp::bls12381::VerifyingKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VerifyingKey%3CG1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/api.rs.html#73-134\">source</a><a href=\"#impl-VerifyingKey%3CG1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;G1: <a class=\"trait\" href=\"fastcrypto/groups/trait.Pairing.html\" title=\"trait fastcrypto::groups::Pairing\">Pairing</a>&gt; <a class=\"struct\" href=\"fastcrypto_zkp/groth16/struct.VerifyingKey.html\" title=\"struct fastcrypto_zkp::groth16::VerifyingKey\">VerifyingKey</a>&lt;G1&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.from_arkworks_format\" class=\"method\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/api.rs.html#74-133\">source</a><h4 class=\"code-header\">pub fn <a href=\"fastcrypto_zkp/groth16/struct.VerifyingKey.html#tymethod.from_arkworks_format\" class=\"fn\">from_arkworks_format</a>&lt;const G1_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>, const G2_SIZE: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a>&gt;(\n    bytes: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u8.html\">u8</a>],\n) -&gt; <a class=\"type\" href=\"fastcrypto/error/type.FastCryptoResult.html\" title=\"type fastcrypto::error::FastCryptoResult\">FastCryptoResult</a>&lt;Self&gt;<div class=\"where\">where\n    G1: <a class=\"trait\" href=\"fastcrypto/serde_helpers/trait.ToFromByteArray.html\" title=\"trait fastcrypto::serde_helpers::ToFromByteArray\">ToFromByteArray</a>&lt;G1_SIZE&gt;,\n    &lt;G1 as <a class=\"trait\" href=\"fastcrypto/groups/trait.Pairing.html\" title=\"trait fastcrypto::groups::Pairing\">Pairing</a>&gt;::<a class=\"associatedtype\" href=\"fastcrypto/groups/trait.Pairing.html#associatedtype.Other\" title=\"type fastcrypto::groups::Pairing::Other\">Other</a>: <a class=\"trait\" href=\"fastcrypto/serde_helpers/trait.ToFromByteArray.html\" title=\"trait fastcrypto::serde_helpers::ToFromByteArray\">ToFromByteArray</a>&lt;G2_SIZE&gt;,</div></h4></section></div></details>",0,"fastcrypto_zkp::bls12381::VerifyingKey"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[4612]}