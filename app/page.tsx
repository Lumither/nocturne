export default function Home() {
    let info = new Array(200).fill(<p> test </p>);
    return (
        <main>
            <div className={ 'h-screen' }>
                <div className={ ' overflow-scroll' }>
                    <ul>
                        { info.map((item, id) => (
                            <li key={ id }> { item } </li>
                        )) }
                    </ul>
                </div>
            </div>
        </main>
    );
}
