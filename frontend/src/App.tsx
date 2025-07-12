import { useQuery } from '@tanstack/react-query';
import './App.css';

const App = () => {
  const apiResponse = useQuery({
    queryKey: ['theOnlyApi'],
    queryFn: async () => (await fetch("/api/hey")).json()
  });

  return (
    <div>
      <h1>Testing the API!</h1>
      {
        apiResponse.status === 'error' && <p>Oh NOOOO! an ERRROR!</p>
      }
      {
        apiResponse.status === 'pending' && <p>Loading...</p>
      }
      {
        apiResponse.status === 'success' && <p>Data: {apiResponse.data.text}</p>
      }
    </div>
  );
};

export default App;
