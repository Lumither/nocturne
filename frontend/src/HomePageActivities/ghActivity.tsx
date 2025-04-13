import { GhActivity } from '@/src/HomePageActivities/activity';
import { match } from 'ts-pattern';
import { StarRepo } from '@/src/HomePageActivities/ghActivities/starRepo';
import { GH_USER_NAME } from '@/src/HomePageActivities/constants';


export const getGhActivities: () => Promise<GhActivity[]> = async () => {
    const data = await fetch(`https://api.github.com/users/${ GH_USER_NAME }/events`, {
        headers: {
            'Accept': 'application/vnd.github+json',
            'X-GitHub-Api-Version': '2022-11-28'
        }
    });
    return data
        .json()
        .then((ret: any): GhActivity[] => (
            ret
                .map((res: any) => (
                    match(res)
                        .returnType<GhActivity | undefined>()
                        // star repo
                        .with(
                            { type: 'WatchEvent', payload: { action: 'started' } }
                            , () => (
                                new StarRepo(res.repo.name, res.repo.url, res.created_at)
                            ))
                        // to be continued
                        .otherwise(() => undefined))
                )
                .filter((res: GhActivity | undefined) => res !== undefined)
        ));
};