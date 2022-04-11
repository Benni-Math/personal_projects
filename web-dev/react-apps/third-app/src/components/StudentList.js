import Student from "./Student";

const StudentList = ({ students }) => {
  return (
    <div>
      {students.map((studentInfo) => (
        <Student studentInfo={studentInfo} />
      ))}
    </div>
  );
};

export default StudentList;
