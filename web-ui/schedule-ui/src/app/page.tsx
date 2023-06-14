import "@fontsource/inter/400.css"; // Defaults to weight 400
import Spacer from 'react-spacer';
import Bells from './components/bells';
import CalendarBlock from "./components/blocks";

export default function Dashboard() {  
  return (
    <div>
      <div style={{ marginTop: '68px', marginLeft: '174px'}}>
        <h1 className='text-3xl' style={{ fontFamily: 'inter'}}>Панель управления</h1>
        <h1 className='text-xl' style={{ fontFamily: 'inter', color: '#969696', marginTop: '8px'}}>Открыть версию для студентов</h1>
      </div>

      <div className='flex mt-9 ml-36'>
        <CalendarBlock />

        <div className='bg-white ml-6' style={{ width: '387px', height: '742px', borderRadius: '20px'}}>
          <div className='flex mt-6 ml-6'>
            <h1 style={{ fontSize: '36px'}}>46</h1>
            <Spacer grow='1' />
            <button className='mr-6' style={{ fontFamily: 'inter', fontSize: '13px', border: '1px solid #E8E8E8', borderRadius: '12px', width: '140px', height: '40px'}}>Смотреть все</button>
          </div>
          <h1 className='text-lg ml-6' style={{ fontSize: '16px',fontFamily: 'inter', color: '#969696'}}>Групп всего</h1>
          

        </div>
        <div className='ml-6'>
            <div className='bg-white' style={{ width: '387px', height: '124px', borderRadius: '20px'}}>
              <div className='flex ml-6'>
              <h1 className='mt-6' style={{ fontSize: '36px'}}>14</h1>
                <Spacer grow='1' />
                <button className='mr-6 mt-6' style={{ fontSize: '13px', border: '1px solid #E8E8E8', borderRadius: '12px', width: '140px', height: '40px'}}>Смотреть все</button>
              </div>
              <h1 className='text-lg ml-6' style={{ fontSize: '16px',fontFamily: 'inter', color: '#969696'}}>Преподавателей</h1>

            </div>
            <div className='bg-white mt-6' style={{ width: '387px', height: '124px', borderRadius: '20px'}}>
              <div className='flex ml-6'>
              <h1 className='mt-6' style={{ fontSize: '36px'}}>26</h1>
                <Spacer grow='1' />
                <button className='mr-6 mt-6' style={{ fontSize: '13px', border: '1px solid #E8E8E8', borderRadius: '12px', width: '140px', height: '40px'}}>Смотреть все</button>
              </div>
              <h1 className='text-lg ml-6' style={{ fontSize: '16px',fontFamily: 'inter', color: '#969696'}}>Предметов</h1>

            </div>
            <div className='bg-white mt-6' style={{ width: '387px', height: '446px', borderRadius: '20px'}}>
              <div className='flex ml-6'>
                <h1 className='mt-6' style={{ fontSize: '36px'}}>9</h1>
                  <Spacer grow='1' />
                  <button className='mr-6 mt-6' style={{ fontSize: '13px', border: '1px solid #E8E8E8', borderRadius: '12px', width: '140px', height: '40px'}}>Смотреть все</button>
                </div>
                <h1 className='text-lg ml-6' style={{ fontSize: '16px', fontFamily: 'inter', color: '#969696'}}>Звонков</h1>
                <Bells/>
            </div>
        </div>
      </div>
      
    </div>
  )
}
