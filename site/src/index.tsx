import React from 'react';
import ReactDOM from 'react-dom/client';
import HomePage from './HomePage';
import { ChakraProvider } from '@chakra-ui/react'

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  <ChakraProvider>
    <HomePage />
  </ChakraProvider>
);

