import Calendar from "./calendar";
import Spacer from 'react-spacer';


export default function CalendarBlock() {
  let data = {
    "month": 'Июнь 2023'
  };

  return (
      <div className='bg-white' style={{ width: '798px', height: '742px', borderRadius: '20px'}}>
        <div className='flex' style={{ marginLeft: '23px'}}>
          <h1 className='text-3xl mt-6' style={{ fontFamily: 'inter'}}>{data.month}</h1>
          <Spacer grow='1'/>

          <div className='mt-6'>
            <button className='mr-4' style={{ backgroundColor: '#5445FF', color: 'white', borderRadius: '12px', height: '40px', width: '200px'}}>Добавить расписание</button>
            <button className='mr-6' style={{ backgroundColor: 'white', color: 'black', borderRadius: '12px', height: '40px', width: '220px', border: '1px solid #E8E8E8'}}>Посмотреть расписание</button>
          </div>
        </div>
        <h1 className='text-xl' style={{ fontFamily: 'inter', color: '#969696', marginTop: '4px', marginLeft: '23px'}}>Расписание на месяц</h1>
        <Calendar />
      </div>
  )
}