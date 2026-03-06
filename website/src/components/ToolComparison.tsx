import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import CodeBlock from '@theme/CodeBlock';
import React from 'react';

type Props = {
  broskiCode: string;
  makeCode: string;
  justCode: string;
  broskiLanguage?: string;
  makeLanguage?: string;
  justLanguage?: string;
};

export default function ToolComparison({
  broskiCode,
  makeCode,
  justCode,
  broskiLanguage = 'bash',
  makeLanguage = 'makefile',
  justLanguage = 'makefile',
}: Props): JSX.Element {
  return (
    <Tabs groupId="tool-compare" queryString>
      <TabItem value="broski" label="Broski">
        <CodeBlock language={broskiLanguage}>{broskiCode}</CodeBlock>
      </TabItem>
      <TabItem value="make" label="Make">
        <CodeBlock language={makeLanguage}>{makeCode}</CodeBlock>
      </TabItem>
      <TabItem value="just" label="Just">
        <CodeBlock language={justLanguage}>{justCode}</CodeBlock>
      </TabItem>
    </Tabs>
  );
}
