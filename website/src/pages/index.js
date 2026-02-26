import React, {useState} from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Translate, {translate} from '@docusaurus/Translate';

export default function Home() {
  const {siteConfig} = useDocusaurusContext();

  return (
    <Layout
      title="forge"
      description={translate({
        id: 'homepage.meta.description',
        message: 'competitive programming, without the boilerplate.',
      })}>
      <main>
        <Hero />
        <Terminal />
        <Features />
        <Install />
      </main>
    </Layout>
  );
}

function Hero() {
  const [copied, setCopied] = useState(false);

  const copyInstall = () => {
    navigator.clipboard.writeText('cargo install forge-cp');
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section className="hero">
      <div className="container">
        <h1 className="hero__title">forge</h1>
        <p className="hero__subtitle">
          <Translate id="homepage.tagline">
            competitive programming, without the boilerplate.
          </Translate>
        </p>

        <div className="install-command" onClick={copyInstall} title="click to copy">
          <code>$ cargo install forge-cp</code>
          {copied && <span style={{marginLeft: '0.5rem', color: '#10b981'}}> copied!</span>}
        </div>

        <div style={{marginTop: '1.5rem', display: 'flex', gap: '1rem', justifyContent: 'center'}}>
          <Link className="button button--primary button--lg" to="/docs/intro">
            <Translate id="homepage.cta.getStarted">get started</Translate>
          </Link>
          <Link className="button button--outline button--lg" href="https://github.com/afonp/forge">
            github
          </Link>
        </div>
      </div>
    </section>
  );
}

function Terminal() {
  return (
    <section className="container" style={{padding: '2rem 0'}}>
      <div className="terminal">
        <div><span className="prompt">$</span> forge new cf1900 a b c d</div>
        <div className="success">[✓] created: ./cf1900_a</div>
        <div className="success">[✓] created: ./cf1900_b</div>
        <div className="success">[✓] created: ./cf1900_c</div>
        <div className="success">[✓] created: ./cf1900_d</div>
        <div className="success">[✓] committed: add contest cf1900 (a, b, c, d)</div>
      </div>
    </section>
  );
}

function Features() {
  const features = [
    {
      title: translate({id: 'homepage.feature.scaffold.title', message: 'instant scaffolding'}),
      desc: translate({
        id: 'homepage.feature.scaffold.desc',
        message: 'every exercise gets solution.cpp, Makefile, input.txt, and notes.md — ready to go.',
      }),
    },
    {
      title: translate({id: 'homepage.feature.template.title', message: 'c++ template'}),
      desc: translate({
        id: 'homepage.feature.template.desc',
        message: 'built-in template with graph, dsu, segtree, fenwick tree, kmp, z-function, and more.',
      }),
    },
    {
      title: translate({id: 'homepage.feature.git.title', message: 'auto git commits'}),
      desc: translate({
        id: 'homepage.feature.git.desc',
        message: 'every action is automatically staged and committed. your progress is always tracked.',
      }),
    },
    {
      title: translate({id: 'homepage.feature.crossplatform.title', message: 'cross-platform'}),
      desc: translate({
        id: 'homepage.feature.crossplatform.desc',
        message: 'works on macos, linux, and windows. single static binary, no runtime dependencies.',
      }),
    },
  ];

  return (
    <section className="container" style={{padding: '2rem 0'}}>
      <h2 style={{textAlign: 'center'}}>
        <Translate id="homepage.features.heading">features</Translate>
      </h2>
      <div className="features-grid">
        {features.map((f, i) => (
          <div className="feature-card" key={i}>
            <h3>{f.title}</h3>
            <p>{f.desc}</p>
          </div>
        ))}
      </div>
    </section>
  );
}

function Install() {
  return (
    <section className="container" style={{padding: '2rem 0 4rem'}}>
      <h2 style={{textAlign: 'center'}}>
        <Translate id="homepage.install.heading">install</Translate>
      </h2>
      <div style={{maxWidth: '600px', margin: '0 auto'}}>
        <pre style={{padding: '1rem', borderRadius: '8px'}}>
          <code>{`# install via cargo
cargo install forge-cp

# create a single exercise
forge new two-sum

# create a contest
forge new cf1900 a b c d

# list exercises
forge list

# open in editor
forge open two-sum

# clean binaries
forge clean two-sum`}</code>
        </pre>
      </div>
    </section>
  );
}
