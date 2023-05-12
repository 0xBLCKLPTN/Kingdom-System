import React from 'react';
import "./styles/HomePage.css";

import { Center, AbsoluteCenter, Text, HStack, VStack, Box, Heading} from '@chakra-ui/react'

function HomePage() {
  return (
    <VStack bgColor="#3073B5" paddingTop="50vh" color='white'>
      <Box>
        <Heading fontFamily='Jua' fontSize='48'>Kingdom-System</Heading>
        <Text color="#CCF6FF">Fast, open-source application for lesson management</Text>
      </Box>
    </VStack>
    
    
  );
}

export default HomePage;
