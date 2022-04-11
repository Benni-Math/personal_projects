import { useState } from "react";

const Student = ({ studentInfo }) => {
  const [showInfo, setShowInfo] = useState(false);

  const toggleInfo = () => {
    setShowInfo(!showInfo);
  };

  const name = `${studentInfo.firstName} ${studentInfo.lastName}`;

  return (
    <div role="button" className="profile" onClick={toggleInfo}>
      <h1 className="profile-name">{name}</h1>
      <img src={studentInfo.image} className="profile-picture" />
      {showInfo && (
        <div>
          <p>{`Email: ${studentInfo.email}`}</p>
          <p>{`Favorite Hobby: ${studentInfo.hobby}`}</p>
        </div>
      )}
    </div>
  );
};

export default Student;
