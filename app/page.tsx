export default function Home() {
    let info = new Array(200).fill(<p> test </p>);
    return (
        <main>
            <ul>
                { info.map((item, id) => (
                    <li key={ id }> { item } </li>
                )) }
            </ul>
        </main>
    );
}
