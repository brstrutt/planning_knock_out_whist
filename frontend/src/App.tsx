import './App.css';
import { useCallback, useMemo, useRef, type JSX } from 'react';
import { v4 as uuidv4 } from 'uuid';
import * as api from './api';
import { UsersList } from './users';

import { useMutation, useQueryClient, useSuspenseQuery } from '@tanstack/react-query';
import type { User } from './api/users';

const App = () => {
  const session_uuid = useSessionUuid();

  const myUser = useSuspenseQuery({
    queryKey: ['users', 'me'],
    queryFn: () => api.users.create(session_uuid)
  }).data;

  return (
    <div>
      <Header />
      <div>Current Name: {myUser.name}</div>
      <NameInputField session_uuid={session_uuid} user={myUser} />
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

function NameInputField(props: { session_uuid: string, user: User }): JSX.Element {
  const { session_uuid, user } = props;
  const messageInput = useRef<HTMLInputElement>(null);

  const queryClient = useQueryClient();
  const setName = useMutation({
    mutationFn: (newName: string) => api.users.update(session_uuid, { ...user, name: newName }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['users'] }),
  });
  const submit = useCallback(
    () => setName.mutate(messageInput.current?.value ?? 'Error: Name was set but input field could not be found!'),
    [setName],
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
