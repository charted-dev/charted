/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { type FC, useEffect, useState, PropsWithChildren } from 'react';
import { keyframes, styled } from '~/stitches.config';
import { Command } from 'cmdk';
import * as Dialog from '@radix-ui/react-dialog';
import { blackA } from '@radix-ui/colors';

const showContent = keyframes({
  '0%': {
    opacity: 0,
    transform: 'translate(-50%, -48%) scale(.96)'
  },

  '100%': {
    opacity: 1,
    transform: 'translate(-50%, -50%) scale(1)'
  }
});

const overlay = keyframes({
  '0%': {
    opacity: 0
  },

  '100%': {
    opacity: 1
  }
});

const Overlay = styled(Dialog.Overlay, {
  backgroundColor: blackA.blackA9,
  position: 'fixed',
  inset: 0,

  noMotion: {
    animation: `${overlay} 150ms cubic-bezier(0.16, 1, 0.3, 1)`
  }
});

const Content = styled(Dialog.Content, {});

/*
const StyledContent = styled(DialogPrimitive.Content, {
  backgroundColor: 'white',
  borderRadius: 6,
  boxShadow: 'hsl(206 22% 7% / 35%) 0px 10px 38px -10px, hsl(206 22% 7% / 20%) 0px 10px 20px -15px',
  position: 'fixed',
  top: '50%',
  left: '50%',
  transform: 'translate(-50%, -50%)',
  width: '90vw',
  maxWidth: '450px',
  maxHeight: '85vh',
  padding: 25,
  '@media (prefers-reduced-motion: no-preference)': {
    animation: `${contentShow} 150ms cubic-bezier(0.16, 1, 0.3, 1)`,
  },
  '&:focus': { outline: 'none' },
});
*/

const CommandMenu: FC = () => {
  const [open, setOpen] = useState(false);
  useEffect(() => {
    const onPress = (event: KeyboardEvent) => {
      console.log(event);
      if (event.key === '/') {
        setOpen(!open);
      }
    };

    document.addEventListener('keydown', onPress);
    return () => document.removeEventListener('keydown', onPress);
  }, []);

  return (
    <Dialog.Portal>
      <Overlay />
    </Dialog.Portal>
  );
};

/*
<Command.Dialog open={open} onOpenChange={setOpen} label="Command Menu">
      <Command.Input />
      <Command.List>
        <Command.Empty>No results were found :(</Command.Empty>
        <Command.Item>Apple</Command.Item>
        <Command.Group heading="beeps and boops?">
          <Command.Item>a</Command.Item>
          <Command.Item>b</Command.Item>
          <Command.Separator />
          <Command.Item>c</Command.Item>
        </Command.Group>
      </Command.List>
    </Command.Dialog>
*/

export default CommandMenu;
