import { GhActivity } from '@/src/HomePageActivities/activity';
import { Card, CardBody } from '@nextui-org/card';
import Link from 'next/link';

export class StarRepo extends GhActivity {
    constructor(
        readonly repoName: string,
        readonly repoUrl: string,
        time: Date
    ) {
        super(time);
    }

    toElement() {
        return (
            // <div>
            //     <Link
            //         href={ this.repoUrl }
            //     >{ `${ GH_USER_NAME } started ${ this.repoName } at ${ this.time }` }</Link>
            // </div>
            <Card
                isHoverable={ true }
                as={ Link }
                href={ this.repoUrl }
                className={ 'bg-black/20' }
            >
                <CardBody>
                    <p className={ 'text-sm' }>{ `Stared GitHub repository '${ this.repoName }'` }</p>
                </CardBody>
            </Card>

        );
    }
}
