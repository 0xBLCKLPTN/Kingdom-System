
export default function Bells() {
    const bells: string[] = ['7:20 - 8:20', '8:30 - 9:50', '10:10 - 11:30', '12:00 - 13:20', '13:40 - 15:00', '15:20 - 16:40', '16:50 - 18:10']
    return (
        <ul className='ml-6' style={{ marginTop: '20px'}}>
            { bells.map((bell, index) => (
                <li className="flex" style={{ borderBottom: '1px solid #E8E8E8', marginRight: '24px', marginTop: '16px'}}>
                    <h1 style={{color: '#969696'}}>{index}.</h1>
                    <h1 style={{ marginLeft: '8px'}}>{bell}</h1>
                </li>
            ))}
        </ul>
    );
}