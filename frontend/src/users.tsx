import { useMemo, type JSX } from "react";
import * as api from './api';
import { useSuspenseQuery } from "@tanstack/react-query";

export function UsersList(props: { excludeIds?: number[] }): JSX.Element {
    const { excludeIds = [] } = props;
    const users = useSuspenseQuery({
        queryKey: ['users'],
        queryFn: () => api.users.list()
    }).data;

    const filteredUsers = useMemo(() => users.filter(user => !excludeIds.includes(user.id)), [users, excludeIds]);

    return <div className='UsersList'>
        {
            filteredUsers.map(
                user => <UserCard key={user.id} user={user} />
            )
        }
    </div>;
}

function UserCard(props: { user: api.users.User }): JSX.Element {
    const { user } = props;
    return (
        <div className='userCard'>{user.name}</div>
    );
}
