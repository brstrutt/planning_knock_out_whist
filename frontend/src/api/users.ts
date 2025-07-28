export type User = {
  id: number;
  name: string;
};

export const users_query_key = ['users'];

export async function create(uuid: string): Promise<User> {
  const response = await fetch('/api/users', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ uuid }),
  });
  return response.json() as Promise<User>;
}

export async function list(): Promise<User[]> {
  const response = await fetch('/api/users', {
    method: 'GET',
  });
  return response.json() as Promise<User[]>;
}

export async function update(uuid: string, newUser: User): Promise<User> {
  const response = await fetch(`/api/users/${newUser.id}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ uuid, name: newUser.name }),
  });
  return response.json() as Promise<User>;
}
