<div align="center">
  <div>
    <img src="https://raw.githubusercontent.com/fuyutarow/piqel/alpha/docs/static/img/label.png">
  </div>
  <strong>An implementation of PartiQL written in Rust</strong>
  <h3>
    <a href="https://piqel.pages.dev">Document(WIP)</a>
  </h3>
</div>
<pre class="code-block"><code class="language-toml:tests-make/hello.toml toml:tests-make/hello.toml">[tests.hello]
script = '''
cat&lt;&lt;EOS | pq -q &quot;SELECT NAME, LOGNAME&quot; -t json
{
  &quot;SHELL&quot;: &quot;/bin/bash&quot;,
  &quot;NAME&quot;: &quot;my machine name&quot;,
  &quot;PWD&quot;: &quot;/home/fuyutarow/piqel&quot;,
  &quot;LOGNAME&quot;: &quot;fuyutarow&quot;,
  &quot;HOME&quot;: &quot;/home/fuyutarow&quot;,
  &quot;LANG&quot;: &quot;C.UTF-8&quot;,
  &quot;USER&quot;: &quot;fuyutarow&quot;,
  &quot;HOSTTYPE&quot;: &quot;x86_64&quot;,
  &quot;_&quot;: &quot;/usr/bin/env&quot;
}
EOS
'''
tobe = '''
[
  {
    &quot;NAME&quot;: &quot;my machine name&quot;,
    &quot;LOGNAME&quot;: &quot;fuyutarow&quot;
  }
]
'''
</code></pre>
<h2 id="family">Family</h2>
<table>
<thead>
<tr>
<th>content</th>
<th>lang</th>
<th>package</th>
</tr>
</thead>
<tbody>
<tr>
<td><a href="https://github.com/fuyutarow/piqel/blob/alpha/src/bin/pq.rs">pq</a></td>
<td>CLI (brew, scoop)</td>
<td></td>
</tr>
<tr>
<td><a href="https://github.com/fuyutarow/piqel">piqel</a></td>
<td>Rust (cargo)</td>
<td><a href="https://crates.io/crates/piqel">https://crates.io/crates/piqel</a></td>
</tr>
<tr>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/piqel-js">piqel-js</a></td>
<td>JavaScript (npm)</td>
<td><a href="https://www.npmjs.com/package/piqel">https://www.npmjs.com/package/piqel</a></td>
</tr>
<tr>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/piqel-py">piqel-py</a></td>
<td>Python (pip)</td>
<td><a href="https://pypi.org/project/piqel">https://pypi.org/project/piqel</a></td>
</tr>
</tbody>
</table>
<h2 id="table-of-contants">Table of Contants</h2>
<ul>
<li><a href="#Features">Features</a></li>
<li><a href="#Motivation">Motivation</a></li>
<li><a href="#Usage">Usage</a>
<ul>
<li><a href="#pretty-print">pretty print</a></li>
<li><a href="#convert-file-format">convert file format</a></li>
<li><a href="#calculate-BMI">calculate BMI</a></li>
</ul>
</li>
<li><a href="#Installation">Installation</a></li>
<li><a href="#Test">Test</a></li>
<li><a href="#LICENCE">LICNECE</a></li>
</ul>
<h2 id="features">Features</h2>
<h2 id="motivation">Motivation</h2>
<p>What’s <a href="https://partiql.org/">PartiQL</a>?</p>
<h2 id="usage">Usage</h2>
<h3 id="pretty-print">pretty print</h3>
<table>
<thead>
<tr>
<th>option</th>
<th>description</th>
</tr>
</thead>
<tbody>
<tr>
<td>-c, --compact</td>
<td>compact instead of pretty-printed output, only when outputting in JSON</td>
</tr>
<tr>
<td>-S, --sort-keys</td>
<td>sort keys of objects on output. it on works when --to option is json, currently</td>
</tr>
</tbody>
</table>
<pre class="code-block"><code class="language-sh:$ sh:$">curl -s &quot;https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1&quot; | pq
</code></pre>
<h3 id="convert-file-format">convert file format</h3>
<table>
<thead>
<tr>
<th>option</th>
<th>description</th>
</tr>
</thead>
<tbody>
<tr>
<td>-f, --from <from></from></td>
<td>target config file [possible values: csv, json, toml, yaml, xml]</td>
</tr>
<tr>
<td>-t, --to <to></to></td>
<td>target config file [possible values: csv, json, toml, yaml, xml]</td>
</tr>
</tbody>
</table>
<p>use <code>-t</code> option c to convert Json, Yaml, Toml, and XML to each other.</p>
<pre class="code-block"><code class="language-sh:$ sh:$">cat pokemon.json | pq -t yaml
</code></pre>
<pre class="code-block"><code class="language-sh:$ sh:$">cat pokemon.json | pq -t yaml | pq -t toml
</code></pre>
<p>Comparison with existing command yj<sup class="footnote-ref"><a href="#fn1" id="fnref1">[1]</a></sup></p>
<table>
<thead>
<tr>
<th>format</th>
<th>pq</th>
<th>yj</th>
</tr>
</thead>
<tbody>
<tr>
<td>JSON</td>
<td>✅</td>
<td>✅</td>
</tr>
<tr>
<td>TOML</td>
<td>✅</td>
<td>⚠️*1</td>
</tr>
<tr>
<td>YAML</td>
<td>✅</td>
<td>✅</td>
</tr>
<tr>
<td>XML</td>
<td>✅</td>
<td>✅</td>
</tr>
<tr>
<td>CSV</td>
<td>✅</td>
<td>❌</td>
</tr>
</tbody>
</table>
<p>*1 TOML of the following format cannot be serialized with <code>yj</code>, but it can be serialized with <code>pq</code> by replacing the fields accordingly.</p>
<pre class="code-block"><code class="language-json:pakcge.json json:pakcge.json">{
  &quot;name&quot;: &quot;partiql-pokemon&quot;,
  &quot;dependencies&quot;: {
    &quot;react&quot;: &quot;^16.13.1&quot;,
    &quot;react-dom&quot;: &quot;^16.13.1&quot;
  },
  &quot;license&quot;: &quot;MIT&quot;
}
</code></pre>
<table>
<thead>
<tr>
<th>option</th>
<th>description</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>-q</code></td>
<td>クエリ</td>
</tr>
</tbody>
</table>
<table>
<thead>
<tr>
<th>query</th>
<th>description</th>
</tr>
</thead>
<tbody>
<tr>
<td><code>SELECT &lt;field_path&gt;</code></td>
<td></td>
</tr>
<tr>
<td><code>SELECT &lt;field_path&gt; AS &lt;alias_path&gt;</code></td>
<td></td>
</tr>
</tbody>
</table>
<h3 id="calculate-bmi">Calculate BMI</h3>
<ol>
<li>Download the file and then calculate BMI in a local.</li>
</ol>
<pre class="code-block"><code class="language-sh:$ sh:$">curl -s https://raw.githubusercontent.com/fuyutarow/pokemon.json/master/en/pokemon.json | pq -q &quot;SELECT name, weight/height/height AS bmi ORDER BY bmi DESC LIMIT 20&quot;
</code></pre>
<ol start="2">
<li>In a terminal, send a query to the server to calculate BMI in a remote.</li>
</ol>
<pre class="code-block"><code class="language-sh:$ sh:$">curl https://partiql-pokemon.vercel.app/api/pokemon/ja -G --data-urlencode &quot;q= SELECT name, weight/height/height AS bmi ORDER BY bmi DESC LIMIT 20&quot;
</code></pre>
<ol start="3">
<li>In a web browser, send a query to the server to calculate BMI in a remote.</li>
</ol>
<a href="https://partiql-pokemon.vercel.app/api/pokemon/ja?q=%20SELECT%20name,%20weight/height/height%20AS%20%20bmi%20ORDER%20BY%20bmi%20DESC%20LIMIT%2020">
partiql-pokemon.vercel.app/api/pokemon/en?q= SELECT name, weight/height/height AS  bmi ORDER BY bmi DESC LIMIT 20
</a>
<h2 id="installation">Installation</h2>
<pre class="code-block"><code class="language-sh:$ sh:$">brew install fuyutarow/tap/pq
pq -h
</code></pre>
<pre class="code-block"><code class="language-sh:$ sh:$">scoop install pq
pq -h
</code></pre>
<h3 id="convert-data">Convert data</h3>
<pre class="code-block"><code class="language-">env | jo | pq &quot;SELECT NAME AS name, USER AS user&quot;
</code></pre>
<p><code>ip</code> command is only available in Linux and WSL, not in Mac.</p>
<pre class="code-block"><code class="language-">ip -j -p | pq &quot;$(cat&lt;&lt;EOS
SELECT
  address,
  info.family AS inet,
  info.local
FROM addr_info AS info
WHERE inet LIKE 'inet%'
EOS
)&quot;
</code></pre>
<ul>
<li>[x] SELECT</li>
<li>[x] FROM</li>
<li>[x] LEFT JOIN</li>
<li>[x] WHERE</li>
<li>[x] LIKE</li>
<li>[x] ORDER BY</li>
<li>[x] LIMIT</li>
</ul>
<p><a href="https://github.com/fuyutarow/piqel/tree/alpha/tests-make">more examples</a></p>
<h2 id="test">Test</h2>
<p>Use <a href="https://github.com/fuyutarow/tests-make">tests-make</a> to test CLI <code>pq</code>.</p>
<pre class="code-block"><code class="language-sh:$ sh:$">brew install fuyutarow/tap/tests-make
tests-make tests-make/index.toml
</code></pre>
<p>or</p>
<pre class="code-block"><code class="language-sh:$ sh:$">makers test:pq
</code></pre>
<table>
<thead>
<tr>
<th>content</th>
<th>test</th>
<th>command</th>
</tr>
</thead>
<tbody>
<tr>
<td><a href="https://github.com/fuyutarow/piqel/blob/alpha/src/bin/pq.rs">pq</a></td>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/tests-make">test</a></td>
<td><code>makers test:pq</code></td>
</tr>
<tr>
<td><a href="https://github.com/fuyutarow/piqel">piqel</a></td>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/tests">test</a></td>
<td><code>makers test:lib</code></td>
</tr>
<tr>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/piqel-js">piqel-js</a></td>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/piqel-js/tests">test</a></td>
<td><code>makers test:js</code></td>
</tr>
<tr>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/piqel-py">piqel-py</a></td>
<td><a href="https://github.com/fuyutarow/piqel/tree/alpha/piqel-py/tests">test</a></td>
<td><code>makres test:py</code></td>
</tr>
<tr>
<td>all</td>
<td></td>
<td><code>makers test</code></td>
</tr>
</tbody>
</table>
<h2 id="code-coverage">code coverage</h2>
<pre class="code-block"><code class="language-sh: sh:">cargo install cargo-kcov
cargo kcov
</code></pre>
<p>or</p>
<pre class="code-block"><code class="language-sh:$ sh:$">makers cov
</code></pre>
<h3 id="preparation">Preparation</h3>
<pre class="code-block"><code class="language-">makers install-dev
</code></pre>
<h3 id="build">build</h3>
<pre class="code-block"><code class="language-">makers build
makers build:pq ;: for pq commnad
</code></pre>
<h2 id="licence">LICENCE</h2>
<h2 id="appendix">Appendix</h2>
<h3 id="comparison-of-tools-that-can-extract-fields">Comparison of tools that can extract fields</h3>
<p>jq<sup class="footnote-ref"><a href="#fn2" id="fnref2">[2]</a></sup> approach</p>
<pre class="code-block"><code class="language-sh:$ sh:$">curl -s &quot;https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1&quot; | jq  &quot;.[].commit.author&quot;
</code></pre>
<p>gron<sup class="footnote-ref"><a href="#fn3" id="fnref3">[3]</a></sup> approach</p>
<pre class="code-block"><code class="language-sh:$ sh:$">curl -s &quot;https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1&quot; | gron | grep &quot;commit.author&quot; | gron -u
</code></pre>
<p>nusehll<sup class="footnote-ref"><a href="#fn4" id="fnref4">[4]</a></sup> approach</p>
<pre class="code-block"><code class="language-sh:nu$ sh:nu$">curl -s &quot;https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1&quot; | from json | get commit.author | to json
</code></pre>
<p>pq<sup class="footnote-ref"><a href="#fn5" id="fnref5">[5]</a></sup> approach</p>
<pre class="code-block"><code class="language-sh:$ sh:$">curl -s &quot;https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1&quot; | pq -q &quot;SELECT commit.author&quot;
</code></pre>
<h3 id="utils">utils</h3>
<ul>
<li>makers<sup class="footnote-ref"><a href="#fn6" id="fnref6">[6]</a></sup></li>
</ul>
<hr class="footnotes-sep">
<section class="footnotes">
<ol class="footnotes-list">
<li id="fn1" class="footnote-item"><p><a href="https://github.com/sclevine/yj">https://github.com/sclevine/yj</a> <a href="#fnref1" class="footnote-backref">↩︎</a></p>
</li>
<li id="fn2" class="footnote-item"><p><a href="https://github.com/stedolan/jq">https://github.com/stedolan/jq</a> <a href="#fnref2" class="footnote-backref">↩︎</a></p>
</li>
<li id="fn3" class="footnote-item"><p><a href="https://github.com/tomnomnom/gron">https://github.com/tomnomnom/gron</a> <a href="#fnref3" class="footnote-backref">↩︎</a></p>
</li>
<li id="fn4" class="footnote-item"><p><a href="https://github.com/nushell/nushell">https://github.com/nushell/nushell</a> <a href="#fnref4" class="footnote-backref">↩︎</a></p>
</li>
<li id="fn5" class="footnote-item"><p><a href="https://github.com/fuyutarow/piqel">https://github.com/fuyutarow/piqel</a> <a href="#fnref5" class="footnote-backref">↩︎</a></p>
</li>
<li id="fn6" class="footnote-item"><p><a href="https://github.com/sagiegurari/cargo-make">https://github.com/sagiegurari/cargo-make</a> … Run <code>cargo install cargo-make</code> to use <code>makers</code> commnad. <a href="#fnref6" class="footnote-backref">↩︎</a></p>
</li>
</ol>
</section>

