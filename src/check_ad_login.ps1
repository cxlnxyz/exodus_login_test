param (
    [string]$username,
    [string]$password,
    [switch]$listUsers
)

if ($listUsers) {
    try {
        $users = Get-ADUser -Filter * -Property SamAccountName, PasswordLastSet
        foreach ($user in $users) {
            Write-Output "Username: $($user.SamAccountName), PasswordLastSet: $($user.PasswordLastSet)"
        }
        exit 0
    } catch {
        Write-Output "Failed to list users"
        exit 1
    }
} else {
    $securePassword = ConvertTo-SecureString $password -AsPlainText -Force
    $credential = New-Object System.Management.Automation.PSCredential ($username, $securePassword)

    try {
        $null = (New-Object System.DirectoryServices.DirectorySearcher).FindOne()
        Write-Output "Login successful"
        exit 0
    } catch {
        Write-Output "Login failed"
        exit 1
    }
}