(function() {var type_impls = {
"fastcrypto_zkp":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Proof%3CG1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/mod.rs.html#13\">source</a><a href=\"#impl-Debug-for-Proof%3CG1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;G1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"fastcrypto/groups/trait.Pairing.html\" title=\"trait fastcrypto::groups::Pairing\">Pairing</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"fastcrypto_zkp/groth16/struct.Proof.html\" title=\"struct fastcrypto_zkp::groth16::Proof\">Proof</a>&lt;G1&gt;<div class=\"where\">where\n    G1::<a class=\"associatedtype\" href=\"fastcrypto/groups/trait.Pairing.html#associatedtype.Other\" title=\"type fastcrypto::groups::Pairing::Other\">Other</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/mod.rs.html#13\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","fastcrypto_zkp::bls12381::Proof"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-Proof%3CG1%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/mod.rs.html#13\">source</a><a href=\"#impl-Deserialize%3C'de%3E-for-Proof%3CG1%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, G1&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"fastcrypto_zkp/groth16/struct.Proof.html\" title=\"struct fastcrypto_zkp::groth16::Proof\">Proof</a>&lt;G1&gt;<div class=\"where\">where\n    G1::<a class=\"associatedtype\" href=\"fastcrypto/groups/trait.Pairing.html#associatedtype.Other\" title=\"type fastcrypto::groups::Pairing::Other\">Other</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,\n    G1: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; + <a class=\"trait\" href=\"fastcrypto/groups/trait.Pairing.html\" title=\"trait fastcrypto::groups::Pairing\">Pairing</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto_zkp/groth16/mod.rs.html#13\">source</a><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(__deserializer: __D) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, __D::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"https://docs.rs/serde/1.0.193/serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","fastcrypto_zkp::bls12381::Proof"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()