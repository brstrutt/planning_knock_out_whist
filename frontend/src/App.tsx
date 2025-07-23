import { useMutation, useQuery } from '@tanstack/react-query';
import './App.css';
import { useCallback, useMemo, useRef } from 'react';
import { v4 as uuidv4 } from 'uuid';

const App = () => {
  const getData = useQuery({
    queryKey: ['theOnlyApi'],
    queryFn: async () => (await fetch('/api/hey')).json(),
  });

  const setData = useMutation({
    mutationFn: async (newText: string) =>
      await fetch('/api/hey', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ text: newText }),
      }),
    onSuccess: () => getData.refetch(),
  });

  const session_uuid = useMemo(() => uuidv4(), []);
  const session = useQuery({
    queryKey: ['session'],
    queryFn: async () => (
      await fetch(
        '/api/connect',
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ session_uuid }),
        }
      )
    ).json(),
  });

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
