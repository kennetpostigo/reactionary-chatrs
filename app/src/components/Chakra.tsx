import { ChakraProvider, localStorageManager } from "@chakra-ui/react";
import { ReactNode } from "react";
import theme from "../lib/theme";

interface ChakraProps {
  cookies?: string;
  children: ReactNode;
}

export const Chakra = ({ children }: ChakraProps) => {
  return (
    <ChakraProvider
      resetCSS
      theme={theme}
      colorModeManager={localStorageManager}
    >
      {children}
    </ChakraProvider>
  );
};
