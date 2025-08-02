import { useMemo, type JSX } from "react";
import * as api from './api';
import { useSuspenseQuery } from "@tanstack/react-query";

export function Sessions(props: { excludeIds?: number[] }): JSX.Element {
    const { excludeIds = [] } = props;
    const users = useSuspenseQuery({
        queryKey: ['users'],
        queryFn: () => api.users.list()
    }).data;

    const filteredUsers = useMemo(() => users.filter(user => !excludeIds.includes(user.id)), [users, excludeIds]);

    return <div className='Sessions'>
        {
            filteredUsers.map(
                user => <Session key={user.id} user={user} />
            )
        }
    </div>;
}

function Session(props: { user: api.users.User }): JSX.Element {
    const { user } = props;
    return (
        <div className='session'>{user.name}</div>
    );
}
