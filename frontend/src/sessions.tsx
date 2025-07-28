import type { JSX } from "react";
import * as api from './api';
import { useSuspenseQuery } from "@tanstack/react-query";

export function Sessions(): JSX.Element {
    const users = useSuspenseQuery({
        queryKey: ['users'],
        queryFn: () => api.users.list()
    }).data;

    return <div className='Sessions'>
        {
            users.map(
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
