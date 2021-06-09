import Head from 'next/head';

import { OGP } from 'models/ogp';

export const MetaHead: React.FC<{
  ogp?: OGP | undefined
}> = ({ ogp }) => {
  const {
    title, url, description, twitterCard, locale, siteName, imageUrl,
  } = {
    title: 'AdsKITA',
    url: 'https://adskita.now.sh',
    description: 'AdsKITA',
    twitterCard: 'summary',
    locale: 'ja_JP',
    siteName: 'AdsKITA',
    ...ogp,
  };

  const og = {
    title,
    description,
    type: 'website',
    siteName,
    locale,
    url,
    imageUrl,
  };

  const twitter = {
    card: twitterCard,
  };

  return (
    <Head>
      <title>{title}</title>
      {/* <meta name="viewport" content="minimum-scale=1, initial-scale=1, width=device-width" /> */}
      <link rel="icon" href="/favicon.ico" />
      <link {... {
        rel: 'canonical',
        href: url,
      }} />
      <meta property="og:type" content={og.type} />
      <meta property="og:site_name" content={og.siteName} />
      <meta property="og:locale" content={og.locale} />
      <meta {...{
        property: 'og:url',
        content: og.url,
      }} />
      <meta {...{
        property: 'og:title',
        content: og.title,
      }} />
      <meta {...{
        property: 'og:description',
        content: og.description,
      }} />
      {og.imageUrl &&
        <meta {...{
          property: 'og:image',
          content: og.imageUrl,
        }} />
      }
      {twitter.card &&
        <meta {...{
          property: 'twitter:card',
          content: twitter.card,
        }} />
      }
    </Head>
  );
};
