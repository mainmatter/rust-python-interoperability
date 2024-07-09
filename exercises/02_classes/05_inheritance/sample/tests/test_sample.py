# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from inheritance import Person, Employee

def test_employee():
    employee = Employee("John", "Doe", 165)
    assert employee.first_name == "John"
    assert employee.last_name == "Doe"
    assert employee.id == 165

    assert employee.full_name() == "John Doe"

def test_person():
    person = Person("Jane", "Smith")
    assert person.first_name == "Jane"
    assert person.last_name == "Smith"

    assert person.full_name() == "Jane Smith"

def test_relationship():
    assert issubclass(Employee, Person)