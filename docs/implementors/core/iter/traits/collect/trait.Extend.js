(function() {var implementors = {};
implementors["either"] = [{"text":"impl&lt;L, R, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt; for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":["either::Either"]}];
implementors["im"] = [{"text":"impl&lt;K, V, RK, RV&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.tuple.html\">(</a>RK, RV<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"im/ordmap/struct.OrdMap.html\" title=\"struct im::ordmap::OrdMap\">OrdMap</a>&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RK&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RV&gt;,&nbsp;</span>","synthetic":false,"types":["im::ord::map::OrdMap"]},{"text":"impl&lt;A, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;R&gt; for <a class=\"struct\" href=\"im/ordset/struct.OrdSet.html\" title=\"struct im::ordset::OrdSet\">OrdSet</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;R&gt;,&nbsp;</span>","synthetic":false,"types":["im::ord::set::OrdSet"]},{"text":"impl&lt;K, V, S, RK, RV&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.tuple.html\">(</a>RK, RV<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"im/hashmap/struct.HashMap.html\" title=\"struct im::hashmap::HashMap\">HashMap</a>&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RK&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;RV&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,&nbsp;</span>","synthetic":false,"types":["im::hash::map::HashMap"]},{"text":"impl&lt;A, S, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;R&gt; for <a class=\"struct\" href=\"im/hashset/struct.HashSet.html\" title=\"struct im::hashset::HashSet\">HashSet</a>&lt;A, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;R&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,&nbsp;</span>","synthetic":false,"types":["im::hash::set::HashSet"]},{"text":"impl&lt;A:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt; for <a class=\"struct\" href=\"im/struct.Vector.html\" title=\"struct im::Vector\">Vector</a>&lt;A&gt;","synthetic":false,"types":["im::vector::Vector"]}];
implementors["sized_chunks"] = [{"text":"impl&lt;A, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt; for <a class=\"struct\" href=\"sized_chunks/inline_array/struct.InlineArray.html\" title=\"struct sized_chunks::inline_array::InlineArray\">InlineArray</a>&lt;A, T&gt;","synthetic":false,"types":["sized_chunks::inline_array::InlineArray"]},{"text":"impl&lt;'a, A, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.reference.html\">&amp;'a </a>A&gt; for <a class=\"struct\" href=\"sized_chunks/inline_array/struct.InlineArray.html\" title=\"struct sized_chunks::inline_array::InlineArray\">InlineArray</a>&lt;A, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>","synthetic":false,"types":["sized_chunks::inline_array::InlineArray"]},{"text":"impl&lt;A, N&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;A&gt; for <a class=\"struct\" href=\"sized_chunks/sized_chunk/struct.Chunk.html\" title=\"struct sized_chunks::sized_chunk::Chunk\">Chunk</a>&lt;A, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"sized_chunks/types/trait.ChunkLength.html\" title=\"trait sized_chunks::types::ChunkLength\">ChunkLength</a>&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":["sized_chunks::sized_chunk::Chunk"]},{"text":"impl&lt;'a, A, N&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.reference.html\">&amp;'a </a>A&gt; for <a class=\"struct\" href=\"sized_chunks/sized_chunk/struct.Chunk.html\" title=\"struct sized_chunks::sized_chunk::Chunk\">Chunk</a>&lt;A, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"sized_chunks/types/trait.ChunkLength.html\" title=\"trait sized_chunks::types::ChunkLength\">ChunkLength</a>&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":["sized_chunks::sized_chunk::Chunk"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()