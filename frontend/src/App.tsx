import { useMutation, useQuery } from '@tanstack/react-query';
import './App.css';

const App = () => {
  const getData = useQuery({
    queryKey: ['theOnlyApi'],
    queryFn: async () => (await fetch('/api/hey')).json()
  });

  const setData = useMutation({
    mutationFn: async (newText: string) => (await fetch(
      '/api/hey',
      {
        method: 'POST',
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({text: newText})
      }
    )),
    onSuccess: () => getData.refetch()
  });

  return (
    <div>
      <h1>Testing the API!</h1>
      {
        getData.status === 'error' && <p>Oh NOOOO! an ERRROR!</p>
      }
      {
        getData.status === 'pending' && <p>Loading...</p>
      }
      {
        getData.status === 'success' && <p>Data: {getData.data.text}</p>
      }

      <button onClick={() => setData.mutate('NEW TEXT')}>
        CLICK ME
      </button>
    </div>
  );
};

export default App;
