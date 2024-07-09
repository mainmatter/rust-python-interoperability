# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from parent import Account, AccountWithHistory

def test_account():
    account = Account(100)
    assert account.balance == 100
    account.balance -= 50
    assert account.balance == 50

def test_account_with_history():
    account = AccountWithHistory(100)
    assert account.balance == 100
    account.balance -= 50
    account.balance += 25
    assert account.balance == 75
    assert account.history == [100, 50]

    # `history` can't be modified directly
    try:
        account.history.append(0)
    except AttributeError:
        pass