import './App.css';
import { useCallback, useMemo, useRef } from 'react';
import { v4 as uuidv4 } from 'uuid';
import * as api from './api';

const App = () => {
  const getData = api.hey.useGet();

  const session_uuid = useMemo(() => uuidv4(), []);
  const session = api.session.useConnect(session_uuid);
  const allSessions = api.session.useList();
  const setName = api.session.useSetName();

  const messageInput = useRef<HTMLInputElement>(null);

  const submit = useCallback(
    () => setName.mutate({ session_uuid: session_uuid, name: messageInput.current?.value ?? 'Error: could not find input element' }),
    [session_uuid, setName],
  );

  return (
    <div>
      <h1>Testing the API!</h1>
      {getData.status === 'error' && <p>Oh NOOOO! an ERRROR!</p>}
      {getData.status === 'pending' && <p>Loading...</p>}
      {getData.status === 'success' && <p>Persistent Message: {getData.data.text}</p>}
      <form
        onSubmit={submit}
      >
        <input ref={messageInput} />
        <input type='button' value='submit' onClick={submit} />
      </form>
      <div>{session.data?.session_status} for uuid {session_uuid}</div>
      {
        allSessions.data?.sessions.map(
          session => <div key={session_uuid}>Session: {session.name}</div>
        )
      }
    </div>
  );
};

export default App;
