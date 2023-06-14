
export default function Calendar() {
    const dates: number[] = [27,28,29,30,31,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,1,2,3,4,5,6];
    const weds: string[] = ['Пн', 'Вт', 'Ср', 'Чт', 'Пт', 'Сб', 'Вс'];
    return (
        <div className='mt-6 ml-6' style={{ width: '750px', height: '594px', border: '1px solid #E8E8E8', borderRadius: '12px'}}>
            <div className='grid grid-cols-7 grid-rows-1' style={{marginTop: '12px', marginLeft: '12px'}}>
              {weds.map((wed) => (
                <h1 style={{ fontSize: '16px'}}>{wed}</h1>
              ))}
            </div>
            
            <div className='grid grid-cols-7 grid-rows-7' style={{ marginTop: '12px'}}>
              {dates.map((date) => (
                <div style={{ borderTop: '1px solid #E8E8E8', borderRight: '1px solid #E8E8E8', height: '91px', width: '107'}}>
                  <h1 style={{marginLeft: '12px', marginTop: '12px', fontSize: '16px'}}>{date}</h1>
                </div>
              ))}
            </div>
          </div>
    );
}
