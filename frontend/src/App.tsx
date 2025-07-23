import './App.css';
import { useCallback, useMemo, useRef } from 'react';
import { v4 as uuidv4 } from 'uuid';
import * as api from './api';

const App = () => {
  const getData = api.hey.useGet();
  const setData = api.hey.usePost();

  const session_uuid = useMemo(() => uuidv4(), []);
  const session = api.connect.usePost(session_uuid);

  const messageInput = useRef<HTMLInputElement>(null);

  const submit = useCallback(
    () => setData.mutate(messageInput.current?.value ?? 'Error: could not find input element'),
    [setData],
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
      <div>{JSON.stringify(session.data)}</div>
    </div>
  );
};

export default App;
