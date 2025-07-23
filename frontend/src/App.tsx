import './App.css';
import { useCallback, useMemo, useRef, type JSX } from 'react';
import { v4 as uuidv4 } from 'uuid';
import * as api from './api';
import { Sessions } from './sessions';

const App = () => {
  const session_uuid = useSessionUuid();

  api.session.useConnect(session_uuid);

  return (
    <div>
      <Header />
      <PersistentMessage />
      <NameInputField session_uuid={session_uuid} />
      <Sessions />
    </div>
  );
};

function useSessionUuid(): string {
  return useMemo(
    () => {
      const current_session_uuid = sessionStorage.getItem('pkow_session_uuid');
      if (current_session_uuid !== null) return current_session_uuid;

      const new_uuid = uuidv4();
      sessionStorage.setItem('pkow_session_uuid', new_uuid);
      return new_uuid;
    },
    []
  );
}

function Header(): JSX.Element {
  return <h1>Testing the API!</h1>;
}

function PersistentMessage(): JSX.Element {
  const getData = api.hey.useGet();
  return <>
    {getData.status === 'error' && <p>Oh NOOOO! an ERRROR!</p>}
    {getData.status === 'pending' && <p>Loading...</p>}
    {getData.status === 'success' && <p>Persistent Message: {getData.data.text}</p>}
  </>;
}

function NameInputField(props: { session_uuid: string }): JSX.Element {
  const { session_uuid } = props;
  const messageInput = useRef<HTMLInputElement>(null);

  const setName = api.session.useSetName();
  const submit = useCallback(
    () => setName.mutate({ session_uuid: session_uuid, name: messageInput.current?.value ?? 'Error: could not find input element' }),
    [session_uuid, setName],
  );

  return <>
    <form
      onSubmit={submit}
    >
      <input ref={messageInput} />
      <input type='button' value='submit' onClick={submit} />
    </form>
  </>;
}

export default App;
