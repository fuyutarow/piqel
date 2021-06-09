import { NextPage } from 'next';
import React, { useState, useEffect, useContext, useRef, useCallback } from 'react';

import View from 'views/Home';
import { MetaHead } from 'components/Header';
import { OGP } from 'models/ogp';

const Page: NextPage<{
  ogp: OGP
}> = ({ ogp }) => {

  return (
    <>
      <MetaHead {...{ ogp }} />
      <View />
    </>
  );
};

Page.getInitialProps = async ({ req }) => {
  const ogp: OGP = {
    title: 'partiql-pokemon',
    url: 'https://partiql.vercel.app',
    description: 'partiql-pokemon',
    // imageUrl: req ? `//${req.headers.host}${icons[512]}` : icons[512],
  };

  return { ogp };
};

export default Page;

