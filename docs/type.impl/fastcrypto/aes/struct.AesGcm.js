(function() {var type_impls = {
"fastcrypto":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AesGcm%3CKeySize,+Aes,+NonceSize%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#265-273\">source</a><a href=\"#impl-AesGcm%3CKeySize,+Aes,+NonceSize%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;KeySize: ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, Aes, NonceSize&gt; <a class=\"struct\" href=\"fastcrypto/aes/struct.AesGcm.html\" title=\"struct fastcrypto::aes::AesGcm\">AesGcm</a>&lt;KeySize, Aes, NonceSize&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#266-272\">source</a><h4 class=\"code-header\">pub fn <a href=\"fastcrypto/aes/struct.AesGcm.html#tymethod.new\" class=\"fn\">new</a>(key: <a class=\"type\" href=\"fastcrypto/aes/type.AesKey.html\" title=\"type fastcrypto::aes::AesKey\">AesKey</a>&lt;KeySize&gt;) -&gt; Self</h4></section></div></details>",0,"fastcrypto::aes::Aes128Gcm","fastcrypto::aes::Aes256Gcm"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Cipher-for-AesGcm%3CKeySize,+Aes,+NonceSize%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#314-332\">source</a><a href=\"#impl-Cipher-for-AesGcm%3CKeySize,+Aes,+NonceSize%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;KeySize: ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, Aes, NonceSize&gt; <a class=\"trait\" href=\"fastcrypto/aes/trait.Cipher.html\" title=\"trait fastcrypto::aes::Cipher\">Cipher</a> for <a class=\"struct\" href=\"fastcrypto/aes/struct.AesGcm.html\" title=\"struct fastcrypto::aes::AesGcm\">AesGcm</a>&lt;KeySize, Aes, NonceSize&gt;<span class=\"where fmt-newline\">where\n    Aes: KeySizeUser&lt;KeySize = KeySize&gt; + KeyInit + BlockCipher + BlockSizeUser&lt;BlockSize = <a class=\"type\" href=\"https://docs.rs/typenum/1.16.0/typenum/generated/consts/type.U16.html\" title=\"type typenum::generated::consts::U16\">U16</a>&gt; + BlockEncrypt,\n    NonceSize: ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.IVType\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.IVType\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"fastcrypto/aes/trait.Cipher.html#associatedtype.IVType\" class=\"associatedtype\">IVType</a> = <a class=\"struct\" href=\"fastcrypto/aes/struct.GenericByteArray.html\" title=\"struct fastcrypto::aes::GenericByteArray\">GenericByteArray</a>&lt;NonceSize&gt;</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.encrypt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#325-327\">source</a><a href=\"#method.encrypt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/aes/trait.Cipher.html#tymethod.encrypt\" class=\"fn\">encrypt</a>(&amp;self, iv: &amp;Self::<a class=\"associatedtype\" href=\"fastcrypto/aes/trait.Cipher.html#associatedtype.IVType\" title=\"type fastcrypto::aes::Cipher::IVType\">IVType</a>, plaintext: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;</h4></section></summary><div class='docblock'>Encrypt <code>plaintext</code> using the given IV and return the result.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decrypt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#329-331\">source</a><a href=\"#method.decrypt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/aes/trait.Cipher.html#tymethod.decrypt\" class=\"fn\">decrypt</a>(\n    &amp;self,\n    iv: &amp;Self::<a class=\"associatedtype\" href=\"fastcrypto/aes/trait.Cipher.html#associatedtype.IVType\" title=\"type fastcrypto::aes::Cipher::IVType\">IVType</a>,\n    ciphertext: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, <a class=\"enum\" href=\"fastcrypto/error/enum.FastCryptoError.html\" title=\"enum fastcrypto::error::FastCryptoError\">FastCryptoError</a>&gt;</h4></section></summary><div class='docblock'>Decrypt <code>ciphertext</code> using the given IV and return the result. An error may be returned in\nCBC-mode if the ciphertext is not correctly padded, but in other modes this method always\nreturn Ok.</div></details></div></details>","Cipher","fastcrypto::aes::Aes128Gcm","fastcrypto::aes::Aes256Gcm"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AuthenticatedCipher-for-AesGcm%3CKeySize,+Aes,+NonceSize%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#275-312\">source</a><a href=\"#impl-AuthenticatedCipher-for-AesGcm%3CKeySize,+Aes,+NonceSize%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;KeySize: ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, Aes, NonceSize&gt; <a class=\"trait\" href=\"fastcrypto/aes/trait.AuthenticatedCipher.html\" title=\"trait fastcrypto::aes::AuthenticatedCipher\">AuthenticatedCipher</a> for <a class=\"struct\" href=\"fastcrypto/aes/struct.AesGcm.html\" title=\"struct fastcrypto::aes::AesGcm\">AesGcm</a>&lt;KeySize, Aes, NonceSize&gt;<span class=\"where fmt-newline\">where\n    Aes: KeySizeUser&lt;KeySize = KeySize&gt; + KeyInit + BlockCipher + BlockSizeUser&lt;BlockSize = <a class=\"type\" href=\"https://docs.rs/typenum/1.16.0/typenum/generated/consts/type.U16.html\" title=\"type typenum::generated::consts::U16\">U16</a>&gt; + BlockEncrypt,\n    NonceSize: ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.IVType\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.IVType\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"fastcrypto/aes/trait.AuthenticatedCipher.html#associatedtype.IVType\" class=\"associatedtype\">IVType</a> = <a class=\"struct\" href=\"fastcrypto/aes/struct.GenericByteArray.html\" title=\"struct fastcrypto::aes::GenericByteArray\">GenericByteArray</a>&lt;NonceSize&gt;</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.encrypt_authenticated\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#287-294\">source</a><a href=\"#method.encrypt_authenticated\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/aes/trait.AuthenticatedCipher.html#tymethod.encrypt_authenticated\" class=\"fn\">encrypt_authenticated</a>(\n    &amp;self,\n    iv: &amp;Self::<a class=\"associatedtype\" href=\"fastcrypto/aes/trait.AuthenticatedCipher.html#associatedtype.IVType\" title=\"type fastcrypto::aes::AuthenticatedCipher::IVType\">IVType</a>,\n    aad: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>],\n    plaintext: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;</h4></section></summary><div class='docblock'>Encrypt <code>plaintext</code> using the given IV and authentication data and return the result.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decrypt_authenticated\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/aes.rs.html#296-311\">source</a><a href=\"#method.decrypt_authenticated\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/aes/trait.AuthenticatedCipher.html#tymethod.decrypt_authenticated\" class=\"fn\">decrypt_authenticated</a>(\n    &amp;self,\n    iv: &amp;Self::<a class=\"associatedtype\" href=\"fastcrypto/aes/trait.AuthenticatedCipher.html#associatedtype.IVType\" title=\"type fastcrypto::aes::AuthenticatedCipher::IVType\">IVType</a>,\n    aad: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>],\n    ciphertext: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, <a class=\"enum\" href=\"fastcrypto/error/enum.FastCryptoError.html\" title=\"enum fastcrypto::error::FastCryptoError\">FastCryptoError</a>&gt;</h4></section></summary><div class='docblock'>Decrypt <code>ciphertext</code> using the given IV and authentication data and return the result.\nAn error is returned if the authentication data does not match the supplied ciphertext.</div></details></div></details>","AuthenticatedCipher","fastcrypto::aes::Aes128Gcm","fastcrypto::aes::Aes256Gcm"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()