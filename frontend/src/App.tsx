import './App.css';
import { useMemo, type JSX } from 'react';
import { v4 as uuidv4 } from 'uuid';
import * as api from './api';
import { CurrentUserCard, UsersList } from './users';

import { useSuspenseQuery } from '@tanstack/react-query';

const App = () => {
  const session_uuid = useSessionUuid();

  const myUser = useSuspenseQuery({
    queryKey: ['users', 'me'],
    queryFn: () => api.users.create(session_uuid)
  }).data;

  return (
    <div>
      <Header />
      <CurrentUserCard user={myUser} session_uuid={session_uuid} />
      <UsersList excludeIds={[myUser.id]} />
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

export default App;
