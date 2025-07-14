import { useMutation, useQuery } from '@tanstack/react-query';
import './App.css';
import { useRef } from 'react';

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

  const messageInput = useRef<HTMLInputElement>(null);

  return (
    <div>
      <h1>Testing the API!</h1>
      {getData.status === 'error' && <p>Oh NOOOO! an ERRROR!</p>}
      {getData.status === 'pending' && <p>Loading...</p>}
      {getData.status === 'success' && <p>Persistent Message: {getData.data.text}</p>}
      <input ref={messageInput} />
      <button
        onClick={() =>
          setData.mutate(messageInput.current?.value ?? 'Error: could not find input element')
        }
      >
        Update Message
      </button>
    </div>
  );
};

export default App;
