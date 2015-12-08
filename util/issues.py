import gitlab

git = gitlab.Gitlab(host="old-repo-url")
newgit = gitlab.Gitlab(host="new-repo-url", verify_ssl=False)

# idg-frontend: old: id =  1
#               new: id = 19
oldprojectid = 1
newprojectid = 19

def printprojects():
    print("old gitlab:")
    print('\n    '.join(a['path'] + ":    " + str(a['id']) for a in git.getprojects()))
    print("new gitlab:")
    print('\n    '.join(a['path'] + ":    " + str(a['id']) for a in newgit.getprojects()))

def migratelabels():
    labels = git.getlabels(oldprojectid)
    print("Start migration of " + str(len(labels)) + " Labels")

    i = 0
    for l in labels:
        print("    migrating label #%s: %s" % (i, l['name']))
        newgit.createlabel(newprojectid, l['name'], l['color'])
        i+=1

def migratemilestones():
    milestones = git.getmilestones(oldprojectid)
    print(milestones)
    print("Start migration of " + str(len(milestones)) + " Issues")

    i = 0
    for m in milestones:
        print("    migrating milestone #%s: %s" % (i, m['title']))
        try:
            due = m['due_date']
        except Exception:
            due = ''
        try:
            desc = m['description']
        except Exception:
            desc = ''

        if not newgit.createmilestone(newprojectid, title=m['title'], due_date=due, description=desc):
            print("        error")
        
        i+=1

def migrateissues():
    milestones = newgit.getmilestones(newprojectid)
    ms = {}
    for m in milestones:
        ms[m['title']] = m['id']

    users = newgit.getusers(page=1, per_page=100)
    us = {}
    for u in users:
        us[u['name']] = u['id']

    l = 100
    i = 1
    issues = []
    while l == 100:
        new = git.getprojectissues(oldprojectid, page=i, per_page=100)
        issues += new
        l = len(new)
        i+=1
    print("Start migration of " + str(len(issues)) + " Issues")

    inc = 0
    # TODO: Flip List, as the api gives them from newest to oldest, but the import should be oldest to newest
    for i in issues:
        print("    migrating issue #%s: %s" % (inc, i['title']))
        comments = git.getissuewallnotes(oldprojectid, i['id'], page=1, per_page=1000)
        description = "Originally created by " + i['author']['name'] + " on " + i['created_at'] + ":\n\n" + i['description']
        if "milestone" in i and i['milestone'] != None and i['milestone']['title'] in ms:
            milestone = ms[i['milestone']['title']]
        else:
            milestone = ''
        if "assignee" in i and i['assignee'] != None and i['assignee']['name'] in us:
            assignee = us[i['assignee']['name']]
        else:
            assignee=''
        newissue = newgit.createissue(newprojectid, title=i['title'], description=description, milestone_id=milestone, labels=i['labels'], assignee_id=assignee)

        for c in comments:
            description = "Originally created by " + c['author']['name'] + " on " + c['created_at'] + ":\n\n" + c['body']
            newgit.createissuewallnote(newprojectid, newissue['id'], description)
        
        if i['state'] == "closed":
            newgit.editissue(newprojectid, newissue['id'], state_event="close")
        inc+=1
# login
if not git.login(user="username", password="password"):
    print("error logging in")
    exit()
if not newgit.login(user="username", password="password"):
    print("error logging in 2")
    exit()

# migratelabels()
# migratemilestones()
migrateissues()

